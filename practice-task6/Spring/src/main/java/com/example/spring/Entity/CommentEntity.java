package com.example.spring.Entity;

import jakarta.persistence.*;

@Entity
@Table(name = "comment", schema = "actix_db", catalog = "")
public class CommentEntity {
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Id
    @Column(name = "id")
    private int id;
    @Basic
    @Column(name = "comment_detail")
    private String commentDetail;
    @Basic
    @Column(name = "commentator")
    private String commentator;
    @Basic
    @Column(name = "movie_name")
    private String movieName;

    public int getId() {
        return id;
    }

    public void setId(int id) {
        this.id = id;
    }

    public String getCommentDetail() {
        return commentDetail;
    }

    public void setCommentDetail(String commentDetail) {
        this.commentDetail = commentDetail;
    }

    public String getCommentator() {
        return commentator;
    }

    public void setCommentator(String commentator) {
        this.commentator = commentator;
    }

    public String getMovieName() {
        return movieName;
    }

    public void setMovieName(String movieName) {
        this.movieName = movieName;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;

        CommentEntity that = (CommentEntity) o;

        if (id != that.id) return false;
        if (commentDetail != null ? !commentDetail.equals(that.commentDetail) : that.commentDetail != null)
            return false;
        if (commentator != null ? !commentator.equals(that.commentator) : that.commentator != null) return false;
        if (movieName != null ? !movieName.equals(that.movieName) : that.movieName != null) return false;

        return true;
    }

    @Override
    public int hashCode() {
        int result = id;
        result = 31 * result + (commentDetail != null ? commentDetail.hashCode() : 0);
        result = 31 * result + (commentator != null ? commentator.hashCode() : 0);
        result = 31 * result + (movieName != null ? movieName.hashCode() : 0);
        return result;
    }
}
